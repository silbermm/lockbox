defmodule LockboxServerTest do
  use ExUnit.Case
  doctest LockboxServer

  test "greets the world" do
    assert LockboxServer.hello() == :world
  end
end
