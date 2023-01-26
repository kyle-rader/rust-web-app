class AccountsController < ApplicationController
  before_action :authenticate

  def index
    render
  end

  def change_display_name_get
    render
  end

  def change_display_name_post
    display_name_params = params.require(:account).permit(:display_name, :password)
    # current_account.update display_name_params
  end

end