defmodule LockboxServer.Syncronizer do
  use GenServer
  require Logger

  def start_link(args), do: GenServer.start_link(__MODULE__, args, name: __MODULE__)

  def init(_args) do
    # args should be a list of accounts to syncronize
    # this is called remotely by a node that has connected
    state = %{
      successful: [],
      errors: [],
    }
    {:ok, state}
  end

  def handle_call({:sync, lst}, state) do
    # go through each item in the list and try to sync
    # as the item is successfully synced, add it to the success
    # if there is an error, add it to the error list
    state =
      Enum.reduce(lst, state, fn item, acc ->
        # TODO: sync item with local db
        %{state | successful: [item | state.successful]}
      end)
    {:ok, state}
  end
end
