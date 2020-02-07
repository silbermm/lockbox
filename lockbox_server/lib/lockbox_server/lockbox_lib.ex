defmodule Lockbox.Lib do
  use Rustler, otp_app: :lockbox_server, crate: :lockbox_interface

  def add(_arg1, _arg2), do: :erlang.nif_error(:nif_not_loaded)
end
