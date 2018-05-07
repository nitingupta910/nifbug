defmodule NifIo.Native do
  use Rustler, otp_app: :nif_io, crate: :io

  def foo(), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
