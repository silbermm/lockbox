defmodule LockboxServer.Cluster.Connection do
  use GenServer
  alias LockboxServer.DataStore
  alias __MODULE__

  defstruct [:trusted_devices]

  def start_link(args), do: GenServer.start_link(__MODULE__, args, name: __MODULE__)

  @impl true
  def init(_args) do
    # get all trusted devices and set in state
    all_trusted = case DataStore.get_all_trusted_devices() do
      {:ok, { _, public_key }} -> public_key
      _ -> []
    end
    {:ok, %Connection{trusted_devices: all_trusted}}
  end

  def handle_call({:connect, n, pub}, _from, state) do
    if(node_trusted?(n, pub, state)) do
      devices = [{n, pub} | state.trusted_devices]
      state = %Connection{trusted_devices: devices}
      {:reply, Node.connect(n), state}
    else
      IO.inspect("NODE NOT TRUSTED (#{n})", label: "CONNECT_CLIENT")
      {:reply, false, state}
    end
  end

  def handle_cast({:trust, n, pub, replicate?}, state) do
    DataStore.add_trusted_device(n, pub)
    #if (replicate?) do
      #IO.puts("replicating to #{n}")
       #GenServer.cast({Genex.Core.Connection, :b@silbermm}, {:trust, :a@silbermm, "1234", false})
    #end
    st = state.trusted_devices ++ {n, pub}
    {:noreply, %{state | trusted_devices: st}}
  end

  def connect([n], public_key) do
    GenServer.call(__MODULE__, {:connect, n, public_key})
  end

  def trust_node(node_name, public_key, replicate \\ false) do
    GenServer.cast(__MODULE__, {:trust, node_name, public_key, replicate})
  end

  defp node_trusted?(node_name, public_key, %{trusted_devices: trusted_devices} = _state) do
    case DataStore.get_trusted_device(node_name) do
      {:ok, {_, pub_key}} -> public_key == pub_key
      _ -> false
    end
  end
end
