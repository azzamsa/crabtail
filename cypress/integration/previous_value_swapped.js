describe("Use previous value Test", () => {
  it("Visits Crabtail", () => {
    cy.visit("/")

    //cy.pause()

    // make sure user able to use previous value
    // in text area even after swap
    cy.fixture("base").then((base) => {
      // prepare state
      cy.get("#input").within(() => {
        cy.get("textarea").type(base.css.value)
      })
      cy.contains("Go").click()
      cy.get("#swap-btn").click()

      // use previous value
      cy.get("#input").within(() => {
        cy.get("textarea").type("31")
      })
      cy.contains("Go").click()

      cy.get("#output").within(() => {
        cy.get("textarea").should("have.attr", "value", base.css.value + "31")
      })
    })
  })
})
