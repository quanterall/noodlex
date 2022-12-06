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

    assert diff_time < 80
    IO.puts("Time elapsed for single-item read: #{diff_time} ms")
  end

  test "reads `test_for_batched.vcf` correctly when using batched mode" do
    handle = Noodlex.Vcf.get_handle(@test_for_batched_vcf_path)
    header = Noodlex.Vcf.get_header(handle)

    assert header.fileformat.major == 4
    assert header.fileformat.minor == 1

    start_time = :os.system_time(:millisecond)

    records1 = Noodlex.Vcf.get_records(handle, 100_000)

    end_time = :os.system_time(:millisecond)
    diff_time = end_time - start_time

    assert length(records1) == 2588
    assert diff_time < 80
    IO.puts("Time elapsed for batched read: #{diff_time} ms")
  end
end
