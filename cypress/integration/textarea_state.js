describe("TextArea Test", () => {
  it("Visits Crabtail", () => {
    cy.visit("/")

    //cy.pause()

    // make sure label doesn't change if textarea value changed
    cy.fixture("base").then((base) => {
      // type at first area, then at second
      cy.get("#input").within(() => {
        cy.get("textarea").type("1").type("2")
      })
      cy.get("#output").within(() => {
        cy.get("textarea").type("1").type("2")
      })
      // repeat
      cy.get("#input").within(() => {
        cy.get("textarea").type("3").type("4")
      })
      cy.get("#output").within(() => {
        cy.get("textarea").type("3").type("4")
      })

      cy.get("label").contains(base.css.label)
      cy.get("label").contains(base.typed.label)
    })
  })
})
