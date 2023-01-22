class AccountController < ApplicationController
  before_action :authenticate

  def index
    render
  end

  def change_display_name
    render
  end

end
