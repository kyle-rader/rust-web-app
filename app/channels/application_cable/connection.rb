module ApplicationCable
  class Connection < ActionCable::Connection::Base
    identified_by :current_user

    def connect
      self.current_user = find_verified_user
    end

    private

    def find_verified_user
      return reject_unauthorized_connection unless cookies && cookies.encrypted
      session = cookies.encrypted['_automata_games_session']

      return reject_unauthorized_connection unless session

      id = session['account_id']
      if id && verified_user = Account.find_by(id: id)
        verified_user
      else
        reject_unauthorized_connection
      end
    end
  end
end