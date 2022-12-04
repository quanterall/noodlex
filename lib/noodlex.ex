defmodule Noodlex do
  use Rustler, otp_app: :noodlex, crate: "noodlex"

  def get_header(_path), do: :erlang.nif_error(:nif_not_loaded)
end
