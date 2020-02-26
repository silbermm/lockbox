import Config

config :lockbox_server, :secret_key, System.fetch_env!("LOCKBOX_SECRET_KEY")
