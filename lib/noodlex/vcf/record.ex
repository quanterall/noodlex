defmodule Noodlex.Vcf.Record do
  @enforce_keys [
    :alternate_bases,
    :chromosome,
    :filters,
    :format,
    :ids,
    :info,
    :position,
    :quality_score,
    :reference_bases,
    :genotypes
  ]

  defstruct @enforce_keys

  @type filters :: :pass | {:fail, [String.t()]}

  @type t :: %__MODULE__{
          alternate_bases: String.t(),
          chromosome: String.t(),
          filters: atom(),
          format: [String.t()],
          ids: [String.t()],
          info: %{String.t() => String.t()},
          position: integer(),
          quality_score: float(),
          reference_bases: String.t(),
          genotypes: %{String.t() => String.t()}
        }
end
