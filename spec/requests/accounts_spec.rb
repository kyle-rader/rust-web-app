require 'rails_helper'

RSpec.describe "Accounts", type: :request do
  bob_email = "bob@gmail.com"
  bob_password = "astrongpassword"
  bob_display_name = "bob42"

  describe "Account Creation" do
    it "can make a new account" do
      create_account bob_email, bob_password, bob_display_name
      assert_response :redirect
      bob = Account.find_by email: bob_email
      expect(bob[:display_name]).to eq bob_display_name
      expect(bob[:status]).to eq "unverified"
    end

    it "cannot use banned words for display name" do
      create_account bob_email, bob_password, "shit"
      expect(response.status).to eq 422
    end
  end

  def create_account(email, password, display_name)
    post "/register", as: :json, params: {
      login: email,
      'login-confirm': email,
      password: password,
      'password-confirm': password,
      display_name: display_name,
    }
  end
end
