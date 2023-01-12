class AccountController < ApplicationController
  before_action :authenticate

  def index
    render
  end

end
