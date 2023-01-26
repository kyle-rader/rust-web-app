class Account < ApplicationRecord
  include Rodauth::Rails.model
  enum :status, unverified: 1, verified: 2, closed: 3

  has_and_belongs_to_many :player_groups

  # validates :display_name, presence: true
end
