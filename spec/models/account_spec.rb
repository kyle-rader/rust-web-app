require 'rails_helper'

RSpec.describe Account, type: :model do
  def new_account
    Account.new(email: 'bob@gmail.com', password: 'abadpassword', display_name: 'bob42')
  end

  it "starts as unverified" do
    expect(new_account.status).to eq("unverified")
  end

  it "is valid with expected params" do
    expect(new_account).to be_valid
  end

  it "validates precense of email" do
    subject = new_account
    subject.email = nil
    expect(subject).to be_invalid
    expect(subject.errors.messages).to eq({
      :email => ["can't be blank"],
    })
  end

  it "has distinct email" do
    new_account.save
    # Try making a second account with same email
    expect { 
      Account.create(email: 'bob@gmail.com', password: 'abadpassword', display_name: 'alice18')
    }.to raise_error(ActiveRecord::RecordNotUnique)
  end

  it "validates precense of display_name" do
    subject = new_account
    subject.display_name = nil
    expect(subject).to be_invalid
  end

  it "has non-empty disply_name" do
    subject = new_account
    subject.display_name = ""
    expect(subject).to be_invalid
    expect(subject.errors.messages).to eq({
      :display_name => ["can't be blank"],
    })
  end

  it "has distinct display_name" do
    new_account.save
    # Try making a second account with same display_name
    expect { 
      Account.create(email: 'alice@gmail.com', password: 'abadpassword', display_name: 'bob42')
    }.to raise_error(ActiveRecord::RecordNotUnique)
  end
end
