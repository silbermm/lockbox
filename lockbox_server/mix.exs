defmodule LockboxServer.MixProject do
  use Mix.Project

  def project do
    [
      app: :lockbox_server,
      version: "0.1.0",
      elixir: "~> 1.9",
      compilers: [:rustler] ++ Mix.compilers(),
      start_permanent: Mix.env() == :prod,
      rustler_crates: [lockbox_interface: [
        path: rustler_path()
      ]],
      deps: deps(),
      lockfile: Path.expand("mix.lock", __DIR__),
      deps_path: Path.expand("deps", __DIR__),
      build_path: Path.expand("_build", __DIR__),
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger],
      mod: {LockboxServer.Application, []}
    ]
  end

  def rustler_path do
    path = Path.expand("native", __DIR__)
    "#{path}/lockbox_interface"
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      { :rustler, "~> 0.21.0" }
    ]
  end
end
