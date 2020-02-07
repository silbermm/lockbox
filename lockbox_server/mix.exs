defmodule LockboxServer.MixProject do
  use Mix.Project

  def project do
    [
      app: :lockbox_server,
      version: "0.1.0",
      elixir: "~> 1.9",
      compilers: [:rustler] ++ Mix.compilers(),
      start_permanent: Mix.env() == :prod,
      rustler_crates: [lockbox_interface: []],
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger],
      mod: {LockboxServer.Application, []}
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      { :rustler, "~> 0.21.0" }
    ]
  end
end
