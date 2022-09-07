class Words
    @@words_4 = File.new("app/assets/words/4.txt").readlines.map {|w| w.strip }
    
    def self.words_4
        @@words_4
    end
    
    def self.triplet
        @@words_4.sample(3).join('-')
    end

end