class AccountsController < ApplicationController
  before_action :authenticate

  def index
    render
  end

  def change_display_name_get
    render
  end

end