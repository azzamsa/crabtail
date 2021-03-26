describe("Base Test", () => {
  it("Visits Crabtail", () => {
    cy.visit("/")
    //cy.pause()

    // check default state
    cy.fixture("base").then((base) => {
      cy.get("#input").within(() => {
        cy.get("label").contains(base.css.label)
        cy.get("textarea")
          .should("have.attr", "placeholder", base.css.placeholder)
          .should("have.attr", "value", "")
      })

      cy.get("#output").within(() => {
        cy.get("label").contains(base.typed.label)
        cy.get("textarea")
          .should("have.attr", "placeholder", base.typed.placeholder)
          .should("have.attr", "value", "")
      })
    })

    // check if transform works
    cy.fixture("base").then((base) => {
      cy.get("#input").within(() => {
        cy.get("textarea")
          .type(base.css.value)
          .should("have.attr", "value", base.css.value)
      })
      cy.contains("Go").click()

      cy.get("#output").within(() => {
        cy.get("textarea").should("have.attr", "value", base.typed.value)
      })
    })
  })
})
