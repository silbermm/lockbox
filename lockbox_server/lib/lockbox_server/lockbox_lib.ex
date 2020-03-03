defmodule Lockbox.Lib do
  use Rustler, otp_app: :lockbox_server, crate: :lockbox_interface

  def public_key_path(), do: :erlang.nif_error(:nif_not_loaded)
  def nonce_path(), do: :erlang.nif_error(:nif_not_loaded)

  def decrypt(_arg1), do: :erlang.nif_error(:nif_not_loaded)

  def public_key_path!() do
    case public_key_path() do
      {:ok, p} -> System.user_home!() <> p
      {:error, _} -> ""
    end
  end

  def nonce_path!() do
    case nonce_path() do
      {:ok, p} -> System.user_home!() <> p
      {:error, _} -> ""
    end
  end
end
