defmodule LockboxServer.Cluster.Connection do
  use GenServer
  require Logger
  alias LockboxServer.DataStore
  alias Lockbox.Lib

  alias __MODULE__

  defstruct [:trusted_devices, :self_pubkey, :self_nonce]

  def start_link(args), do: GenServer.start_link(__MODULE__, args, name: __MODULE__)

  @impl true
  def init(_args) do
    # set our pubkey and nonce in state
    pub_key = File.read!(Lib.public_key_path!())
    nonce = File.read!(Lib.nonce_path!())

    # get all trusted devices and set in state
    all_trusted = case DataStore.get_all_trusted_devices() do
      {:ok, { _, public_key }} -> public_key
      _ -> []
    end
    {:ok, %Connection{trusted_devices: all_trusted, self_pubkey: pub_key, self_nonce: nonce}}
  end

  def handle_call({:connect, n, pub, nonce}, _from, state) do
    if(node_trusted?(n, pub, state)) do
      Logger.info("node is trusted")
      devices = [{n, pub} | state.trusted_devices]
      state = %Connection{trusted_devices: devices}
      {:reply, Node.connect(n), state}
    else
      # Eventually we may want to do something different 
      # when the node isn't yet trusted.
      # Options include
      #   ask the user to trust the connection
      #   look at a file that has predefined list of trusted clients
      Logger.info("node is NOT trusted")
      GenServer.cast(__MODULE__, {:trust, n, pub, nonce, true})
      {:reply, Node.connect(n), state}
    end
  end

  def handle_cast({:trust, n, pub, nonce, replicate?}, state) do
    Logger.info("adding node to datastore")
    DataStore.add_trusted_device(n, pub, nonce)
    if (replicate?) do
      Logger.info("replicating to #{n}")
      GenServer.cast({__MODULE__, n}, {:trust, Node.self(), "local pub key", "local nonce", false})
    end
    devices = [{n, pub} | state.trusted_devices]
    state = %Connection{trusted_devices: devices}
    {:noreply, state}
  end

  def connect([n], public_key, nonce) do
    GenServer.call(__MODULE__, {:connect, n, public_key, nonce})
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
