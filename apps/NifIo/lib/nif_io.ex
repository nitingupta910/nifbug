defmodule NifIo do
  use Rustler, otp_app: :nif_io, crate: :io

  def graph_foo(), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
