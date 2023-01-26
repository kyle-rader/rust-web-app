Rails.application.routes.draw do
  root 'application#index'
  get 'privacy', to: 'application#privacy'

  get 'chat', to: 'chat#index'

  get 'account', to: 'accounts#index'
  get 'change-display-name', to: 'accounts#change_display_name_get'
  post 'change-display-name', to: 'accounts#change_display_name_post'
end
