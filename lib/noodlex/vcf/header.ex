defmodule Noodlex.Vcf.Header do
  defmodule FileFormat do
    @enforce_keys [:major, :minor]

    defstruct @enforce_keys

    @type t :: %__MODULE__{major: integer(), minor: integer()}
  end

  defmodule Info do
    @enforce_keys [:id, :number, :type_, :description]

    defstruct @enforce_keys

    @type t :: %__MODULE__{
            id: atom(),
            number: atom(),
            type_: atom(),
            description: String.t()
          }
  end

  @enforce_keys [:fileformat, :infos]

  defstruct @enforce_keys

  @type t :: %__MODULE__{fileformat: FileFormat.t()}
end
