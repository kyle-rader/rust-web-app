Rails.application.routes.draw do
  root 'application#index'
  get 'privacy', to: 'application#privacy'

  get 'chat', to: 'chat#index'

  get 'account', to: 'account#index'
  get 'change-display-name', to: 'account#change-display'
end
