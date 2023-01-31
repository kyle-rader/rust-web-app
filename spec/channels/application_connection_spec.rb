require 'rails_helper'

RSpec.describe ApplicationCable::Connection, type: :channel do
  fixtures :accounts

  it "rejects without cookies" do
    expect { connect }.to have_rejected_connection
  end

  it "connects with cookies" do
    user1 = accounts(:user1)

    cookies.encrypted['_automata_games_session'] = {
      'account_id' => user1.id
    }

    connect

    expect(connection.current_user).to eq user1
  end
end
