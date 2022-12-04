defmodule Noodlex.Vcf.Handle do
  @enforce_keys [:header, :stream]

  defstruct @enforce_keys

  @type t :: %__MODULE__{
          header: Noodlex.Vcf.Header.t(),
          stream: Noodlex.Vcf.Stream.t()
        }
end
