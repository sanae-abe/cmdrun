class Cmdrun < Formula
  desc "Fast, secure, and cross-platform command runner with TOML configuration"
  homepage "https://github.com/sanae-abe/cmdrun"
  url "https://github.com/sanae-abe/cmdrun/archive/refs/tags/v1.0.0.tar.gz"
  sha256 "" # Will be calculated after release
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "#{bin}/cmdrun", "--version"
    system "#{bin}/cmdrun", "--help"
  end
end
