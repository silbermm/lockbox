defmodule LockboxServer.Application do
  @moduledoc false

  use Application

  def start(_type, _args) do
    LockboxServer.DataStore.create()
    topologies = %{
        lockbox_server: [
          strategy: LockboxServer.Cluster.Gossip,
          config: [
            port: 45892,
            if_addr: "0.0.0.0",
            multicast_addr: "230.1.1.251",
            multicast_ttl: 1,
            secret: Application.get_env(:lockbox_server, :secret_key)
          ]
        ]}
    children = [
      {Cluster.Supervisor, [topologies, [name: Lockbox.Supervisor]]}, 
      {LockboxServer.Cluster.Connection, []},
      {LockboxServer.Syncronizer, []},
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: LockboxServer.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
