class Jlog < Formula
  desc "Log notes quickly from the terminal"
  homepage "https://github.com/Jakub-Szymkowiak/jlog"
  url "https://github.com/Jakub-Szymkowiak/jlog/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "..." # 
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "#{bin}/jlog", "--help"
  end
end