class PlayerGroup < ApplicationRecord
  has_and_belongs_to_many :accounts
end
