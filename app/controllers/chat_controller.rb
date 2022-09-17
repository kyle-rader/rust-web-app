class ChatController < ApplicationController
    before_action :authenticate

    def index
        @inertia_app = 'main'
        render inertia: 'chat/index', props: { user: current_account.display_name }
    end
end
