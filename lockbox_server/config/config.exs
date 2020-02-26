import Config

config :libcluster,
  debug: true

import_config "#{Mix.env()}.exs"
