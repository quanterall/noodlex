defmodule NoodlexTest do
  use ExUnit.Case
  doctest Noodlex

  test "greets the world" do
    assert Noodlex.hello() == :world
  end
end
