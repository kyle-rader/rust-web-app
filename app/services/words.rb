class Words
    def Words.triplet
        @@words_4.sample(3).join('-')
    end

    private

    @@words_4 = File.new("app/assets/words/4.txt").readlines.map {|w| w.strip }
end