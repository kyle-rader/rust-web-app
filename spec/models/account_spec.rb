require 'rails_helper'

RSpec.describe Account, type: :model do
  subject {
    Account.new(email: 'bob@gmail.com', password: 'abadpassword', alias: 'bob42')
  }

  it "is valid with expected params" do
    expect(subject).to be_valid
  end

  it "starts as unverified" do
    expect(subject.status).to eq("unverified")
  end

  it "has distinct email" do
    subject.save
    expect { Account.create(email: 'bob@gmail.com', password: 'abadpassword', alias: 'alice18') }.to raise_error(ActiveRecord::RecordNotUnique)
  end

  it "has distinct alias" do
    subject.save
    expect { Account.create(email: 'alice@gmail.com', password: 'abadpassword', alias: 'bob42') }.to raise_error(ActiveRecord::RecordNotUnique)
  end
end
