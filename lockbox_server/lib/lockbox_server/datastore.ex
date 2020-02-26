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
      {:atomic, [{@trusted_device_table, name, public_key}]} -> {:ok, {name, public_key}}
      other -> :empty
    end
  end

  # TODO: this query does NOT work!
  def get_all_trusted_devices do
    case :mnesia.transaction(fn -> :mnesia.select(@trusted_device_table, [{:_,[],[:"$_"]}]) end) do
      {:atomic, []} -> :empty
      {:atomic, [_|_] = data} ->
        devices = Enum.map(data, fn {@trusted_device_table, name, public_key} ->
          {name, public_key}
        end)
        {:ok, devices}
      other -> 
        Logger.debug("Got a different result, #{inspect other}")
        :empty
    end
  end

  def add_trusted_device(name, public_key) do
    :mnesia.transaction(fn -> :mnesia.write({@trusted_device_table, name, public_key}) end)
  end

  defp create_trusted_devices_table() do
    :mnesia.create_table(
      @trusted_device_table,
      attributes: [:name, :public_key],
      disc_copies: [node()]
    )
  end
end
