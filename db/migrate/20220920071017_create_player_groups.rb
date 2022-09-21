class CreatePlayerGroups < ActiveRecord::Migration[7.0]
  def change
    create_table :player_groups do |t|
      t.string :name
      t.timestamps
    end

    create_table :accounts_player_groups, id: false do |t|
      t.belongs_to :account
      t.belongs_to :player_group
    end
  end
end
