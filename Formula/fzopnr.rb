class Fzopnr < Formula
  desc "CLI tool to open URLs with fuzzy finder"
  homepage "https://github.com/k-kawa/fzopnr"
  version "0.0.0"

  if OS.mac?
    if Hardware::CPU.arm?
      url "https://github.com/k-kawa/fzopnr/releases/download/v0.0.0/fzopnr-darwin-arm64"
      sha256 "0000000000000000000000000000000000000000000000000000000000000000"
    else
      url "https://github.com/k-kawa/fzopnr/releases/download/v0.0.0/fzopnr-darwin-amd64"
      sha256 "0000000000000000000000000000000000000000000000000000000000000000"
    end
  end

  def install
    bin.install "fzopnr-darwin-arm64" => "fzopnr" if Hardware::CPU.arm?
    bin.install "fzopnr-darwin-amd64" => "fzopnr" if Hardware::CPU.intel?
  end

  test do
    system "#{bin}/fzopnr", "--help"
  end
end
