defmodule Noodlex.VcfTest do
  use ExUnit.Case
  doctest Noodlex.Vcf

  @test_vcf_path :noodlex |> :code.priv_dir() |> Path.join("test.vcf")

  test "reads correct amount of records from `test.vcf`" do
    handle = Noodlex.Vcf.get_handle(@test_vcf_path)
    header = Noodlex.Vcf.get_header(handle)

    assert header.fileformat.major == 4
    assert header.fileformat.minor == 1

    Enum.each(1..2588, fn _i ->
      %Noodlex.Vcf.Record{} = Noodlex.Vcf.get_record(handle)
    end)

    {:error, :end_of_file} = Noodlex.Vcf.get_record(handle)
  end
end
