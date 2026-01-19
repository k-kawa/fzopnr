class Fzopnr < Formula
  desc "CLI tool to open URLs with fuzzy finder"
  homepage "https://github.com/k-kawa/fzopnr"
  version "1.0.0"

  if OS.mac?
    if Hardware::CPU.arm?
      d30e71869595d9b226c1023f469f0773b97d3a0c5aea86f351cd9ac11479dc"
    else
      ac14712a811fdce6e202ccfe8edc55e70398794f809d7e96b8fe369e2883c"
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
