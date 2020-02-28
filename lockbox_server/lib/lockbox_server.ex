defmodule LockboxServer do
  @moduledoc """
  Documentation for LockboxServer.
  """

  def sync_with_peer(peer, public_key) do
    # get list of accounts with passwords encrypted with peers public key
    # send that list over to the peer
    GenServer.call({Syncronizer, peer}, {:sync, []})
  end
end
