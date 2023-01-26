class AccountsController < ApplicationController
  before_action :authenticate

  def index
    render
  end

  def change_display_name
    if request.post?
      display_name_params = params.permit(:display_name, :password, :authenticity_token, :commit)
      if rodauth.password_match?(display_name_params[:password])
        current_account[:display_name] = display_name_params[:display_name]
        if current_account.save
          flash[:notice] = "Display name saved!"
          redirect_to account_path and return
        end
      else
        flash[:alert] = "That password doesn't look right."
      end
    end

    render
  end
end