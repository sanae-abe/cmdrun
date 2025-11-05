# typed: false
# frozen_string_literal: true

# Homebrew Formula for cmdrun
class Cmdrun < Formula
  desc "Fast, secure, and cross-platform command runner with TOML configuration"
  homepage "https://github.com/sanae-abe/cmdrun"
  version "1.0.0"
  license "MIT OR Apache-2.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/sanae-abe/cmdrun/releases/download/v1.0.0/cmdrun-1.0.0-aarch64-apple-darwin.tar.gz"
      sha256 "" # Will be updated after first release
    else
      url "https://github.com/sanae-abe/cmdrun/releases/download/v1.0.0/cmdrun-1.0.0-x86_64-apple-darwin.tar.gz"
      sha256 "" # Will be updated after first release
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/sanae-abe/cmdrun/releases/download/v1.0.0/cmdrun-1.0.0-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "" # Will be updated after first release
    else
      url "https://github.com/sanae-abe/cmdrun/releases/download/v1.0.0/cmdrun-1.0.0-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "" # Will be updated after first release
    end
  end

  def install
    bin.install "cmdrun"

    # Generate shell completions
    output = Utils.safe_popen_read("#{bin}/cmdrun", "completion", "bash")
    (bash_completion/"cmdrun").write output

    output = Utils.safe_popen_read("#{bin}/cmdrun", "completion", "zsh")
    (zsh_completion/"_cmdrun").write output

    output = Utils.safe_popen_read("#{bin}/cmdrun", "completion", "fish")
    (fish_completion/"cmdrun.fish").write output
  end

  test do
    # Test basic execution
    assert_match "cmdrun #{version}", shell_output("#{bin}/cmdrun --version")

    # Test command creation
    (testpath/"commands.toml").write <<~EOS
      [commands.test]
      description = "Test command"
      cmd = "echo 'Hello, cmdrun!'"
    EOS

    assert_match "Hello, cmdrun!", shell_output("#{bin}/cmdrun run test")
  end
end
