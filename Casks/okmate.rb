cask "okmate" do
  version "0.1.0"
  sha256 :no_check

  url "https://github.com/koliyo/okmate/releases/download/v#{version}/Okmate.zip"
  name "Okmate"
  desc "Open knowledge mate for OKF bundles"
  homepage "https://github.com/koliyo/okmate"

  livecheck do
    url "https://github.com/koliyo/okmate/releases/latest"
    strategy :github_latest
  end

  auto_updates true

  app "Okmate.app"
  binary "#{appdir}/Okmate.app/Contents/MacOS/okmate", target: "okmate"
end
