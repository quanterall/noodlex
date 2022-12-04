defmodule Noodlex.Vcf.Stream do
  @enforce_keys [:reader]

  defstruct @enforce_keys

  @type t :: %__MODULE__{reader: Rustler.Term.t()}
end
