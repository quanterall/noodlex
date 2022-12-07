defmodule Noodlex.VcfTest do
  use ExUnit.Case, async: false
  doctest Noodlex.Vcf

  @test_vcf_path :noodlex |> :code.priv_dir() |> Path.join("test.vcf")
  @test_for_batched_vcf_path :noodlex |> :code.priv_dir() |> Path.join("test_for_batched.vcf")

  test "reads correct amount of records from `test.vcf`" do
    handle = Noodlex.Vcf.get_handle(@test_vcf_path)
    header = Noodlex.Vcf.get_header(handle)

    assert header.fileformat.major == 4
    assert header.fileformat.minor == 1

    start_time = :os.system_time(:millisecond)

    Enum.each(1..2588, fn _i ->
      %Noodlex.Vcf.Record{} = Noodlex.Vcf.get_record(handle)
    end)

    {:error, :end_of_file} = Noodlex.Vcf.get_record(handle)

    end_time = :os.system_time(:millisecond)
    diff_time = end_time - start_time

    assert diff_time < 100
    IO.puts("Time elapsed for single-item read: #{diff_time} ms")
  end

  test "reads `test_for_batched.vcf` correctly when using batched mode" do
    handle = Noodlex.Vcf.get_handle(@test_for_batched_vcf_path)
    header = Noodlex.Vcf.get_header(handle)

    assert header.fileformat.major == 4
    assert header.fileformat.minor == 1

    start_time = :os.system_time(:millisecond)

    records1 = read_all_records(handle)

    end_time = :os.system_time(:millisecond)
    diff_time = end_time - start_time

    assert length(records1) == 2588
    assert diff_time < 100
    IO.puts("Time elapsed for batched read: #{diff_time} ms")
  end

  defp get_batch(_handle, batch, 1_000) do
    Enum.reverse(batch)
  end

  defp get_batch(handle, batch, count) do
    case Noodlex.Vcf.get_record(handle) do
      %Noodlex.Vcf.Record{} = record ->
        get_batch(handle, [record | batch], count + 1)

      {:error, :end_of_file} ->
        batch
    end
  end

  defp read_all_records(handle), do: read_all_records(handle, [])

  defp read_all_records(handle, results) do
    case get_batch(handle, [], 0) do
      [] ->
        results

      batch ->
        read_all_records(handle, results ++ batch)
    end
  end
end
