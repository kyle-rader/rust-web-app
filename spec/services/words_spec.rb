require 'rails_helper'

TEST_WORDS = Set.new(["foo", "foobar"])

RSpec.describe "Words" do
  def subject(word)
    Words.banned?(word, TEST_WORDS)
  end

  it { expect(subject("foo")).to be true }
  it { expect(subject("FOo")).to be true }
  it { expect(subject("FOOBAR")).to be true }
  it { expect(subject("foo bar")).to be true }
  it { expect(subject(" foo bar  ")).to be true }
  it { expect(subject("foo-bar")).to be true }
  it { expect(subject("foo_bar")).to be true }

  it { expect(subject("this_is_fine")).to be false }
end