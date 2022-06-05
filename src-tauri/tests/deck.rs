use app::deck::*;

#[tokio::test]
async fn read_deck() {
    let dir = "tests/test-files";
    let template = Template::from_dir(&dir).await.unwrap();
    let mut deck = Deck::from_file(&format!("{}/deck.json", dir)).await.unwrap();

    assert_eq!(1, deck.notes.len());

    fetch_all_audio(&mut deck, &format!("{}/media", dir)).await.unwrap();

    let mut pkg = Package::new(deck, template).await.unwrap();

    assert_eq!(pkg.deck.notes[0].word, "test");

    pkg.write(&format!("{}/out.apkg", dir), &format!("{}/media", dir), None).await.unwrap();
    pkg.to_deck().write(&format!("{}/deck.json", dir)).await.unwrap();
}