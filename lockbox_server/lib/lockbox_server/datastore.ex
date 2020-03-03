defmodule LockboxServer.DataStore do
  require Logger

  @trusted_device_table :trusted_devices

  def create() do
    Logger.debug("setting up datastore on #{node()}")
    :mnesia.create_schema([node()])
    :mnesia.start()
    create_trusted_devices_table()
  end

  def get_trusted_device(name) do
    case :mnesia.transaction(fn -> :mnesia.read({@trusted_device_table, name}) end) do
      {:atomic, []} -> :empty
      {:atomic, [{@trusted_device_table, name, public_key, nonce}]} -> {:ok, {name, public_key, nonce}}
      _other -> :empty
    end
  end

  def get_all_trusted_devices do
    case :mnesia.transaction(fn -> :mnesia.select(@trusted_device_table, [{:_,[],[:"$_"]}]) end) do
      {:atomic, []} -> :empty
      {:atomic, [_|_] = data} ->
        devices = Enum.map(data, fn {@trusted_device_table, name, public_key, nonce} ->
          {name, public_key, nonce}
        end)
        {:ok, devices}
      other -> 
        Logger.error("Got a different result, #{inspect other}")
        :empty
    end
  end

  def add_trusted_device(name, public_key, nonce) do
    :mnesia.transaction(fn -> :mnesia.write({@trusted_device_table, name, public_key, nonce}) end)
  end

  defp create_trusted_devices_table() do
    :mnesia.create_table(
      @trusted_device_table,
      attributes: [:name, :public_key, :nonce],
      disc_copies: [node()]
    )
  end
end
