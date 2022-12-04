defmodule Noodlex.Vcf do
  use Rustler, otp_app: :noodlex, crate: "noodlex"

  def get_handle(_path), do: :erlang.nif_error(:nif_not_loaded)
  def get_header(_handle), do: :erlang.nif_error(:nif_not_loaded)
  def get_record(_handle), do: :erlang.nif_error(:nif_not_loaded)
end
