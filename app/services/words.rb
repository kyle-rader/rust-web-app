class Words
  def self.scrub(word)
    word
    .gsub(/[\s\-_]+/, "") # sub space, dash, underscores with empty string
    .downcase()
  end

  @@words_4 = File.new("app/assets/words/4.txt").readlines.map {|w| w.strip }
  @@banned = Set.new(File.new("app/assets/words/banned.txt").readlines.map {|w| scrub(w.strip) })

  def self.words_4
    @@words_4
  end

  def self.triplet
    @@words_4.sample(3).join('-')
  end

  def self.banned?(word, word_list = @@banned)
    word_list.include?(scrub(word))
  end
end