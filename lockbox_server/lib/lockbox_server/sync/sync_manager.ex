defmodule LockboxServer.Sync.SyncManager do
  use GenServer
  require Logger

  def start_link(args), do: GenServer.start_link(__MODULE__, args, name: __MODULE__)

  def init(_args) do
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
        # get item from the local db with the same account and username
        # if it exists,
        #     check the last modified time of eacn and take the latest one
        #     update the item
        # else
        #   insert the item
        %{state | successful: [item | state.successful]}
      end)
    {:ok, state}
  end
end
